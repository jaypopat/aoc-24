import os
import shutil

def process_cargo_projects():
    cargo_projects = []
    for root, _, files in os.walk('.'):
        if 'Cargo.toml' in files:
            cargo_projects.append(root)

            # Remove .git directory if it exists
            git_dir = os.path.join(root, '.git')
            if os.path.isdir(git_dir):
                print(f"Removing .git directory from: {root}")
                shutil.rmtree(git_dir)
                print(f".git directory removed from: {root}")

            # Update .gitignore
            gitignore_path = os.path.join(root, '.gitignore')
            if not os.path.exists(gitignore_path):
                open(gitignore_path, 'w').close()

            with open(gitignore_path, 'r+') as f:
                content = f.read()
                if 'input.txt' not in content:
                    f.seek(0, 2)  # Move to the end of the file
                    f.write('\ninput.txt\n')

    print(f"Processed {len(cargo_projects)} Cargo projects:")
    print("- Removed .git directories where present")
    print("- Added 'input.txt' to .gitignore files")

if __name__ == "__main__":
    process_cargo_projects()
