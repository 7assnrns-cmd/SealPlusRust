import os
import json
import re

# 1. تحديث tauri.conf.json بـ versionName = "0.1.1"
tauri_path = "src-tauri/tauri.conf.json"
if os.path.exists(tauri_path):
    with open(tauri_path, "r", encoding="utf-8") as f:
        data = json.load(f)
    data["version"] = "0.1.1"
    with open(tauri_path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)
    print("✅ Updated tauri.conf.json version to 0.1.1")

# 2. تحديث Cargo.toml
cargo_path = "src-tauri/Cargo.toml"
if os.path.exists(cargo_path):
    with open(cargo_path, "r", encoding="utf-8") as f:
        content = f.read()
    content = re.sub(r"(^version\s*=\s*\")[^\"]*(\")", r"\10.1.1\2", content, flags=re.MULTILINE)
    with open(cargo_path, "w", encoding="utf-8") as f:
        f.write(content)
    print("✅ Updated Cargo.toml version to 0.1.1")

# 3. تعديل الـ Workflow لضمان ضبط versionCode = 2 و versionName = "0.1.1" في أندرويد
workflow_path = ".github/workflows/build.yml"
if os.path.exists(workflow_path):
    with open(workflow_path, "r", encoding="utf-8") as f:
        wf_content = f.read()
    
    # تحديث تگ الإصدار في الـ Release
    wf_content = wf_content.replace("tag_name: v0.2.0", "tag_name: v0.1.1")
    wf_content = wf_content.replace("v0.2.0 🚀", "v0.1.1 🚀")
    wf_content = wf_content.replace("tag_name: v0.1.0", "tag_name: v0.1.1")
    wf_content.replace("v0.1.0 🚀", "v0.1.1 🚀")

    # إضافة خطوة لضبط versionCode = 2 في build.gradle أو build.gradle.kts
    gradle_patch_step = """
    - name: Set Android Version Code to 2
      run: |
        PYTHON_SCRIPT='
        import glob
        for path in glob.glob("src-tauri/gen/android/**/build.gradle*", recursive=True):
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()
            if "versionCode" in content:
                import re
                content = re.sub(r"versionCode\\s+\\d+", "versionCode 2", content)
                content = re.sub(r"versionCode\\s*=\\s*\\d+", "versionCode = 2", content)
                with open(path, "w", encoding="utf-8") as f:
                    f.write(content)
                print(f"Patched versionCode in {path}")
        '
        python3 -c "$PYTHON_SCRIPT"
    """

    if "Set Android Version Code to 2" not in wf_content:
        # حقن الخطوة بعد تهيئة مشروع أندرويد
        target_str = "npx tauri android init || true"
        if target_str in wf_content:
            wf_content = wf_content.replace(target_str, target_str + "\n" + gradle_patch_step)

    with open(workflow_path, "w", encoding="utf-8") as f:
        f.write(wf_content)
    print("✅ Updated build.yml workflow for version code 2")

