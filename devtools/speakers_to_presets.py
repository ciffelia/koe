# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "pyyaml",
#     "requests",
# ]
# ///

import argparse
from pathlib import Path

import requests
import yaml

API_URL = "http://voicevox:50021/speakers"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("filename", nargs="?", type=Path)
    args = parser.parse_args()
    preset_file = args.filename

    # Load existing presets
    if preset_file is not None:
        with open(preset_file, encoding="utf-8") as f:
            existing_presets = yaml.safe_load(f)
    else:
        existing_presets = []

    used_ids = {p["id"] for p in existing_presets}
    next_available_id = max(used_ids) + 1 if used_ids else 0

    existing_presets_map = {p["name"]: p for p in existing_presets}

    # Fetch speakers from API and convert to presets
    speakers = get_speakers()
    api_presets = speakers_to_presets(speakers)

    # Merge presets
    final_presets = []

    for api_preset in api_presets:
        preset_name = api_preset["name"]

        if preset_name in existing_presets_map:
            final_presets.append(existing_presets_map[preset_name])
        else:
            final_presets.append(
                {
                    **api_preset,
                    "id": next_available_id,
                }
            )
            next_available_id += 1

    presets_yaml = yaml.safe_dump(
        final_presets,
        allow_unicode=True,
        sort_keys=False,
        default_flow_style=False,
    )
    print(presets_yaml)


def get_speakers():
    response = requests.get(API_URL)
    response.raise_for_status()
    return response.json()


def speakers_to_presets(speakers: list[dict]) -> list[dict]:
    output_list = []

    for speaker in speakers:
        for style in speaker["styles"]:
            new_item = {
                "id": -1,
                "name": f"{speaker['name']} {style['name']}",
                "speaker_uuid": speaker["speaker_uuid"],
                "style_id": style["id"],
                "speedScale": 1.2,
                "pitchScale": 0,
                "intonationScale": 1,
                "volumeScale": 1,
                "prePhonemeLength": 0.1,
                "postPhonemeLength": 0.1,
                "pauseLength": None,
                "pauseLengthScale": 1,
            }
            output_list.append(new_item)

    return output_list


if __name__ == "__main__":
    main()
