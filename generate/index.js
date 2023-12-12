const simpleGit = require("simple-git");
const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");

const repoUrl = process.argv[2];

if (!repoUrl) {
  console.error(
    "Please provide a GitHub repository URL as a command-line argument."
  );
  process.exit(1);
}

const repoName = repoUrl
  .split("/")
  .pop()
  .replace(/\.git$/, "");
const repoPath = path.join(__dirname, repoName);
const sourcePath = path.join(repoPath, "source");

// Clone the repository
console.log(`Cloning repository: ${repoUrl}`);
simpleGit().clone(repoUrl, sourcePath, (err, _) => {
  if (err) {
    console.error(`Error cloning repository: ${err}`);
    process.exit();
  }

  // Get the list of commits
  console.log("Fetching commits...");
  const commitHashes = execSync(`git -C ${sourcePath} log --format=%H::%s`, {
    encoding: "utf-8",
  })
    .trim()
    .split("\n");

  let stepCounter = 1;
  let templateFound = false;
  let solutionFound = false;
  let templateFiles = [];
  let solutionFiles = [];
  let sourceFiles = [];
  let stepNames = [];

  // Create a folder for each commit
  // Reverse to make the oldest commit first
  // Slice to remove the very initial commit
  commitHashes
    .reverse()
    .slice(1)
    .forEach((commitInfo, index) => {
      const [commitHash, commitMessage] = commitInfo.split("::");
      const isTemplate = commitMessage.toLowerCase().startsWith("template: ");
      const isSolution = commitMessage.toLowerCase().startsWith("solution: ");
      const isSection = commitMessage.toLowerCase().startsWith("section: ");

      let stepFolder = path.join(repoPath, stepCounter.toString());
      if (!fs.existsSync(stepFolder)) {
        fs.mkdirSync(stepFolder);
      }

      let sourceFolder = path.join(stepFolder, "source");
      let templateFolder = path.join(stepFolder, "template");
      let solutionFolder = path.join(stepFolder, "solution");

      // Default assumption is output is not a template or solution
      let outputFolder = sourceFolder;

      if (isTemplate) {
        // Check there isn't a template already in queue
        if (templateFound) {
          console.error("A second template was found before a solution.");
          process.exit(1);
        }

        templateFound = true;

        // make step folder
        outputFolder = templateFolder;
      }

      if (isSolution) {
        // Check that there is a template in queue
        if (!templateFound) {
          console.error("No template was found for this solution.");
          process.exit(1);
        }

        // Check that a solution is not already found.
        if (solutionFound) {
          console.error("A second solution was found before a template.");
          process.exit(1);
        }

        solutionFound = true;
        outputFolder = solutionFolder;
      }

      fs.mkdirSync(outputFolder);

      // Checkout the commit
      console.log(`Checking out commit: ${commitHash}`);
      execSync(`git -C ${sourcePath} checkout ${commitHash}`);

      // Copy the contents to the commit folder
      execSync(`cp -r ${sourcePath}/* ${outputFolder}`);
      console.log(`Contents of commit ${index + 1} copied to ${outputFolder}`);

      // Get the list of modified or created files in the commit
      const diffOutput = execSync(
        `git -C ${sourcePath} diff --name-status HEAD~1 HEAD`,
        { encoding: "utf-8" }
      )
        .trim()
        .split("\n");
      const diffRaw = execSync(
        `git -C ${sourcePath} diff HEAD~1 HEAD ':(exclude)README.md'`,
        { encoding: "utf-8" }
      );

      // Create a raw output
      let diff_name = "changes.diff";
      if (isSolution) {
        diff_name = "solution.diff";
      } else if (isTemplate) {
        diff_name = "template.diff";
      }
      const diffFilePath = path.join(outputFolder, diff_name);
      fs.writeFileSync(diffFilePath, diffRaw);

      // Create a JSON file in the commit folder
      const jsonFilePath = path.join(outputFolder, "commit_info.json");
      const commitInfoObject = {
        commitHash,
        commitMessage,
        files: diffOutput.map((line) => {
          const [status, file] = line.split("\t");
          return { status, file };
        }),
      };

      if (isTemplate) {
        templateFiles = commitInfoObject.files;
      } else if (isSolution) {
        solutionFiles = commitInfoObject.files;
      } else {
        sourceFiles = commitInfoObject.files;
      }

      fs.writeFileSync(jsonFilePath, JSON.stringify(commitInfoObject, null, 2));

      // Reset sanity check and increment step
      // Handle when both template and solution is found,
      // or when there is a step that is neither a template or solution
      if (
        (templateFound && solutionFound) ||
        (!templateFound && !solutionFound)
      ) {
        if (templateFound) {
          markdownContent = templateMarkdown;
          let templateFileText = generateFileMarkdown(
            "template",
            templateFiles
          );
          let solutionFileText = generateFileMarkdown(
            "solution",
            solutionFiles
          );
          markdownContent = markdownContent.replace(
            "<!-- insert_template_files -->",
            templateFileText
          );
          markdownContent = markdownContent.replace(
            "<!-- insert_solution_files -->",
            solutionFileText
          );

          let diffText = generateDiffMarkdown("template");
          markdownContent = markdownContent.replace(
            "<!-- insert_diff_files -->",
            diffText
          );

          stepNames.push(getStepName(templateFolder));
        } else {
          markdownContent = sourceMarkdown;
          let sourceFileText = generateFileMarkdown("source", sourceFiles);
          markdownContent = markdownContent.replace(
            "<!-- insert_source_files -->",
            sourceFileText
          );

          let diffText = generateDiffMarkdown("source");
          markdownContent = markdownContent.replace(
            "<!-- insert_diff_files -->",
            diffText
          );

          stepNames.push(getStepName(sourceFolder));
        }
        // Create a Markdown file in the commit folder
        const markdownFilePath = path.join(stepFolder, "README.md");
        fs.writeFileSync(markdownFilePath, markdownContent);
        stepCounter += 1;
        templateFound = false;
        solutionFound = false;
      }
    });

  generateSidebar(stepNames);

  // Clean up source folder
  fs.rmSync(sourcePath, { recursive: true, force: true });

  console.log("Process completed.");
});

// Generate the markdown text for files.
function generateFileMarkdown(type, files) {
  // type is expected to be one of "source", "solution", or "template"
  if (type != "solution" && type != "source" && type != "template") {
    process.exit(1);
  }

  let output = "";
  for (file of files) {
    let filepath = `./${type}/${file.file}`;
    let filename = path.parse(filepath).base;

    // Skip README
    if (filename == "README.md") {
      continue;
    }
    // Skip hidden files
    if (filename.startsWith(".")) {
      continue;
    }
    // Skip Cargo.lock
    if (filename == "Cargo.lock") {
      continue;
    }

    let classStyle = `file-${type}`;
    if (file.status == "M") {
      classStyle += " file-modified";
    } else if (file.status == "A") {
      classStyle += " file-added";
    } else if (file.status == "D") {
      classStyle += " file-deleted";
    }

    let codeStyle = "text";
    let extname = path.extname(filepath);
    if (extname == ".rs") {
      codeStyle = "rust";
    } else if (extname == ".toml") {
      codeStyle = "toml";
    }

    output += `#### **<span class="${classStyle}">${file.file}</span>**\n\n`;
    output += `[${filepath}](${filepath} ':include :type=code ${codeStyle}')\n\n`;
  }

  return output;
}

function generateDiffMarkdown(type) {
  let output = "";

  if (type == "template" || type == "solution") {
    let filepath = `./template/template.diff`;
    output += `#### **template.diff**\n\n`;
    output += `[${filepath}](${filepath} ':include :type=code diff')\n\n`;

    filepath = `./solution/solution.diff`;
    output += `#### **solution.diff**\n\n`;
    output += `[${filepath}](${filepath} ':include :type=code diff')\n\n`;
  } else {
    let filepath = `./${type}/changes.diff`;
    output += `#### **changes.diff**\n\n`;
    output += `[${filepath}](${filepath} ':include :type=code diff')\n\n`;
  }

  return output;
}

let templateMarkdown = `
[filename](./template/README.md ':include')

<!-- slide:break -->

<!-- tabs:start -->

#### **template**

<!-- tabs:start -->

<!-- insert_template_files -->

<!-- tabs:end -->

#### **solution**

<!-- tabs:start -->

<!-- insert_solution_files -->

<!-- tabs:end -->

#### **diff**

<!-- tabs:start -->

<!-- insert_diff_files -->

<!-- tabs:end -->

<!-- tabs:end -->
`;

let sourceMarkdown = `
[filename](./source/README.md ':include')

<!-- slide:break -->

<!-- tabs:start -->

#### **source**

<!-- tabs:start -->

<!-- insert_source_files -->

<!-- tabs:end -->

#### **diff**

<!-- tabs:start -->

<!-- insert_diff_files -->

<!-- tabs:end -->

<!-- tabs:end -->
`;

function getStepName(folder) {
  const filePath = path.join(folder, "README.md");
  const markdownContent = fs.readFileSync(filePath, "utf8");
  const titleMatch = markdownContent.match(/^#\s+(.*)/m);
  if (titleMatch) {
    return titleMatch[1];
  } else {
    console.error(`Error getting markdown title.`);
    process.exit(1);
  }
}

function generateSidebar(steps) {
  const sidebarFilePath = path.join(repoPath, "_sidebar.md");
  let output = "- [Home](/)\n\n---\n\n";
  steps.forEach((step, index) => {
    output += `- [${index + 1}. ${step}](${repoName}/${index + 1}/README.md)\n`;
  });
  fs.writeFileSync(sidebarFilePath, output);
}
