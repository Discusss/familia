import fs from "fs";
import path from "path";

const folder = process.argv[2];

if (!folder) {
  console.error("Please provide a folder name.");
  process.exit(1);
}

if (!fs.existsSync(folder)) {
  console.error(`Folder "${folder}" does not exist.`);
  process.exit(1);
}

// Step 1: Rename files to temporary unique names
const files = fs.readdirSync(folder);
const tempNames = files.map((file, index) => `temp_${index}_${file}`);

for (let i = 0; i < files.length; i++) {
  const file = files[i];
  const tempName = tempNames[i];

  const oldPath = path.join(folder, file);
  const tempPath = path.join(folder, tempName);
  fs.renameSync(oldPath, tempPath);
}

// Step 2: Rename temp files to final names
for (let i = 0; i < tempNames.length; i++) {
  const tempName = tempNames[i];
  const ext = tempName.split('.').pop()?.toLowerCase();
  const newName = `${i + 1}.${ext}`;

  const tempPath = path.join(folder, tempName);
  const newPath = path.join(folder, newName);
  fs.renameSync(tempPath, newPath);
  console.log(`Renamed ${tempName} to ${newName}`);
}

// Verify that there are the exact number of files
const newFiles = fs.readdirSync(folder);
if (newFiles.length !== files.length) {
  console.error(
      `There are ${newFiles.length} files in the folder, but there should be ${files.length}`
  );
  process.exit(1);
}