#!/usr/bin/env bash
# Automated checks for game audio files (draft, tested with ffmpeg 6.1 on 2026-10-22).
# Usage: audio-checks.sh FILE...   Prints one line per file; exits 1 if any file fails.
# Thresholds are starting points, not standards; see README.md section 3.
set -u
MAX_PEAK_DB=-1.0      # sample peak ceiling (dBFS); true peak is reported by ebur128
LEAD_SILENCE_MAX=0.05 # seconds of silence allowed before the sound starts
TAIL_SILENCE_MAX=0.25 # seconds of silence allowed after it ends
SILENCE_DB=-60dB
MIN_DUR=0.03          # seconds
status=0
for f in "$@"; do
  name=$(basename "$f")
  fails=()
  # Naming: category_subject_action_NN.ext, lowercase, no spaces.
  [[ "$name" =~ ^(sfx|mus|vo|amb|ui)_[a-z0-9]+(_[a-z0-9]+)*_[0-9]{2}\.(wav|ogg)$ ]] || fails+=("name")
  dur=$(ffprobe -v error -show_entries format=duration -of csv=p=0 "$f")
  stats=$(ffmpeg -hide_banner -nostats -i "$f" \
    -af "astats=metadata=0,silencedetect=n=${SILENCE_DB}:d=0.01,ebur128=peak=true" -f null - 2>&1)
  peak=$(grep -m1 'Peak level dB' <<<"$stats" | awk '{print $NF}')
  tpk=$(grep -A1 'True peak:' <<<"$stats" | grep -m1 'Peak:' | awk '{print $2}')
  lufs=$(grep -E '^ +I:' <<<"$stats" | tail -1 | awk '{print $2}')
  # silencedetect closes a silence at end of file, so a silence ending at the duration is the tail.
  read -r lead tail allsil < <(grep -oE 'silence_(start|end): [-0-9.e]+' <<<"$stats" | awk -v d="$dur" '
    /start/{s=$2} /end/{e=$2; if (s<=0.001 && e>=d-0.001) a=1; else if (s<=0.001) l=e; else if (e>=d-0.001) t=d-s}
    END{printf "%s %s %s\n", l+0, t+0, a+0}')
  [[ "$peak" == "-inf" ]] && peak=-999
  awk -v p="${peak:--999}" -v m="$MAX_PEAK_DB" 'BEGIN{exit !(p>m)}' && fails+=("peak")
  awk -v d="$dur" -v m="$MIN_DUR" 'BEGIN{exit !(d<m)}' && fails+=("duration")
  if [[ "$allsil" == 1 ]]; then fails+=("all-silent"); else
    awk -v l="$lead" -v m="$LEAD_SILENCE_MAX" 'BEGIN{exit !(l>m)}' && fails+=("lead-silence")
    awk -v t="$tail" -v m="$TAIL_SILENCE_MAX" 'BEGIN{exit !(t>m)}' && fails+=("tail-silence")
  fi
  result=PASS; ((${#fails[@]})) && { result="FAIL(${fails[*]})"; status=1; }
  printf '%s\t%s\tdur=%.3fs\tpeak=%sdBFS\ttruepeak=%sdBFS\tI=%sLUFS\n' "$result" "$name" "$dur" "${peak:-?}" "${tpk:-?}" "${lufs:-?}"
done
exit $status
