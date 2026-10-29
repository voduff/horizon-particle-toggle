#!/usr/bin/env python3
"""Package the bundle as a registry .hnmod: horizon-native.json and the sources."""
import json
from pathlib import Path
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[1]
COMPAT = json.loads((ROOT / "compatibility.json").read_text())


def main():
    (bundle,) = [p.parent for p in (ROOT / "mixins").glob("*/horizon-mixin.toml")]
    mixin = tomllib.loads((bundle / "horizon-mixin.toml").read_text())
    mod_id, version = mixin["mod"]["id"], mixin["mod"]["version"]
    manifest = {
        "schema": 1,
        "id": mod_id,
        "version": version,
        "game": COMPAT["minecraft"],
        "horizon": mixin["compatibility"]["horizon_revision"],
        "mixins": COMPAT["horizon_mixins_revision"],
    }
    out = ROOT / "target" / f"{mod_id}-{version}.hnmod"
    out.parent.mkdir(exist_ok=True)

    def add(zip_file, name, data):
        # Fixed times and regular-file modes keep the file reproducible.
        info = zipfile.ZipInfo(name, date_time=(1980, 1, 1, 0, 0, 0))
        info.external_attr = 0o100644 << 16
        info.compress_type = zipfile.ZIP_DEFLATED
        zip_file.writestr(info, data)

    with zipfile.ZipFile(out, "w") as zip_file:
        add(zip_file, "horizon-native.json", json.dumps(manifest, indent=2) + "\n")
        for path in sorted(bundle.iterdir()):
            if path.suffix in (".rs", ".toml"):
                add(zip_file, f"mixins/{mod_id}/{path.name}", path.read_bytes())
    print(out)


if __name__ == "__main__":
    main()
