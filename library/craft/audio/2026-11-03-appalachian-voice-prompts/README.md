---
title: Appalachian accent prompts for ElevenLabs Voice Design
type: guide
date: 2026-11-03
status: current
projects: []
topics: [audio, voice, elevenlabs, accent, appalachian, prompts]
basis: the Wikipedia article on Appalachian English and ElevenLabs' Voice Design docs, both read 2026-11-03 [S1, S2, S3]; four prompts written, none generated or heard yet
credit: Researched by Claude sessions for GraySun2
---
# Appalachian accent prompts for ElevenLabs Voice Design

Four ready-to-paste ElevenLabs Voice Design prompts for an Appalachian mountain accent, each a voice description plus a matching preview text. The default is an older man from the coal country of southern West Virginia and eastern Kentucky (Foxfire's region in Hallow Season), with a thick but natural accent; the variants are a mountain grandmother, a younger woman with a light modern twang, and a Smoky Mountain man at full strength. The descriptions carry the sounds and the preview texts carry the grammar and words, all taken from the article [S1] and written in our own words. The owner asked for it on 2026-11-03: "use this wiki article to make an eleven labs prompt for the accent I want". The same text is in the Claude Doc [Appalachian accent prompts for ElevenLabs](https://claude.ai/artifact/QTSMAcp7FDUfFu4NNWmczS).

## How to use

1. Paste a voice description into Voice Design, its preview text into "Text to preview", and generate. Each generation gives three voices; only the preview text's characters are charged [S2].
2. Start guidance at about 35%. ElevenLabs says to go higher when accent accuracy matters; its examples run from 20% to 40% [S2].
3. If the accent drifts toward a general Southern or Texas sound, raise guidance or delete "Very good audio quality": quality phrases can make niche voices less accurate [S2].
4. For a voice heard over the radio, keep the voice clean and add the radio sound in the game, or swap the quality words for "Low-fidelity audio." [S2]
5. Lines written for the saved voice should keep the mountain spellings (holler, a-huntin', fixin' to).
6. Ship nothing made on a free plan: ElevenLabs' terms conflict on commercial use ([audio study](../2026-10-22-audio-for-agent-made-games/README.md)).

Preview text must be 100 to 1,000 characters [S3]. The counts below are without the line break.

## Default: old storyteller from coal country (my pick)

Voice description, 753 characters:

```text
English, Southern Appalachian American accent, from the coal country of southern West Virginia and eastern Kentucky. A man in his late 60s. Very good audio quality. Persona: old mountain storyteller. Emotion: warm, wry, unhurried. He has a thick, natural, lived-in mountain accent: "I" and "my" flatten to "ah" and "mah", "fire" sounds like "far", stressed vowels stretch into a drawl so "red" sounds like "ray-ud", "pen" and "pin" sound alike, -ing endings become -in', and "hollow" becomes "holler". His r's are always pronounced. Mid-low pitch, slightly gravelly, with a soft nasal twang. He talks slowly and deliberately, drops the front off words like 'bout and 'member, leans hard on the important word, and pauses before the good part of a story.
```

Preview text, 423 characters (also used by variants 1 and 3):

```text
[chuckles] Well, I reckon I can tell it, but you ain't gonna like it. When I was a young'un, me and my brother went a-huntin' way up the holler, past the old mine. It come on airish right quick, and there was a light a-movin' through them trees, green as foxfire. Hit weren't no lantern, I'll tell you that. Mamaw said it was a haint. Liketa scared me plumb to death. So you'uns might should stay off that ridge after dark.
```

## Variant 1: mountain grandmother

Voice description, 778 characters:

```text
English, Southern Appalachian American accent, from the coal country of southern West Virginia and eastern Kentucky. A woman in her 70s. Very good audio quality. Persona: mountain grandmother telling stories. Emotion: warm, sharp-witted, knowing. She has a thick, natural, lived-in mountain accent: "I" and "my" flatten to "ah" and "mah", "fire" sounds like "far", stressed vowels stretch into a drawl so "red" sounds like "ray-ud", "pen" and "pin" sound alike, -ing endings become -in', and "hollow" becomes "holler". Her r's are always pronounced. Mid pitch, a little thin and creaky with age, with a bright nasal twang. She talks at an easy pace, quickens when she gets to the gossip, drops the front off words like 'bout and 'member, and lowers her voice for the scary part.
```

## Variant 2: younger, light twang

The article notes that old forms such as a-prefixing fade in younger speakers [S1], so this one keeps the vowels and drops most of the rest. Voice description, 577 characters:

```text
English, Appalachian American accent, from a small town in eastern Kentucky. A woman in her late 20s. Very good audio quality. Persona: friendly hometown local. Emotion: easygoing, bright, a little teasing. She has a slight, modern Appalachian twang rather than an old-time one: "I" and "my" lean toward "ah" and "mah", -ing endings become -in', "pen" and "pin" sound alike, and stressed vowels get a light drawl. Her r's are always pronounced. Clear mid pitch with a gentle nasal edge. She talks at a relaxed, normal pace with a friendly lilt, speeding up when she is excited.
```

Preview text, 340 characters:

```text
Yeah, I grew up out in the holler, about twenty minutes past town. Mamaw still lives up there, and she'll feed you till you can't move. If you're headed that way, take the left fork past the church. You'll want a truck, though, that road's plumb washed out after all this rain. I'm fixin' to run up there Saturday if you want to ride along.
```

## Variant 3: Smoky Mountains, full strength

The article places the strongest vowel shift in the Inland South, around Knoxville and Asheville [S1]. Voice description, 786 characters:

```text
English, Smoky Mountain Southern Appalachian American accent, from the mountains of East Tennessee and western North Carolina. A man in his 80s. Very good audio quality. Persona: old-time mountain farmer. Emotion: dry, solemn, quietly funny. He has the thickest, old-time mountain accent, natural and unforced: "time" sounds close to "tahm" and "fire" like "far", "bait" slides toward "bite", stressed vowels break into a long drawl so "feet" sounds like "fih-eet", "pen" and "pin" sound alike, "it" becomes "hit" at the start of a sentence, -ing endings become -in', "hollow" is "holler" and "wash" is "warsh". His r's are always pronounced. Low, weathered, slightly hoarse voice with a nasal twang. He speaks very slowly, with long pauses, stretching the stressed word in each phrase.
```

## What the prompts take from the article

ElevenLabs performs whatever the preview text says [S2], so the grammar and words go there. The article says little about intonation, so the pitch and pacing in the prompts are our choices.

| Feature [S1] | Example | Used in |
| --- | --- | --- |
| Long "i" loses its glide | "time" like "tahm", "fire" like "far" | All descriptions |
| Drawl on stressed vowels | "red" like "ray-ud", "feet" like "fih-eet" | Default, variants 1 and 3 |
| "Bait" slides toward "bite" | the vowel of "bait" starts lower | Variant 3 |
| Pin and pen sound the same | "ink pen" said to tell them apart | All descriptions |
| Every r is pronounced | fully rhotic | All descriptions |
| -ing becomes -in' | huntin', fixin' | All descriptions and both previews |
| Final "-ow" becomes "-er" | holler, yeller | All descriptions and both previews |
| An r added | "wash" like "warsh" | Variant 3 |
| First syllable dropped | 'bout, 'member | Default, variant 1 |
| "Hit" for "it" at the start or for stress | "Hit weren't no lantern" | Variant 3, default preview |
| a- before -in' verbs after go or come | went a-huntin', a-movin' | Default preview |
| Double modals | might should | Default preview |
| Liketa, meaning "nearly" | liketa scared me | Default preview |
| Them for those; me and him as subject | them trees, me and my brother | Default preview |
| Double negatives | weren't no lantern | Default preview |
| Mountain words | reckon, young'un, you'uns, airish (chilly), haint (ghost), plumb (completely), right (very), fixin' to | Both previews |

Not yet known: how well ElevenLabs holds each accent, until the owner generates them.
