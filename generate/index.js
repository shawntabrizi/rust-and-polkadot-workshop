const simpleGit = require('simple-git');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const repoUrl = process.argv[2];

if (!repoUrl) {
	console.error('Please provide a GitHub repository URL as a command-line argument.');
	process.exit(1);
}

const repoName = repoUrl.split('/').pop().replace(/\.git$/, '');
const repoPath = path.join(__dirname, repoName);
const sourcePath = path.join(repoPath, "source");

// Clone the repository
console.log(`Cloning repository: ${repoUrl}`);
simpleGit().clone(repoUrl, sourcePath, (err, _) => {
	if (err) {
		console.error(`Error cloning repository: ${err}`);
		process.exit(1);
	}

	// Get the list of commits
	console.log('Fetching commits...');
	const commitHashes = execSync(`git -C ${sourcePath} log --format=%H::%s`, { encoding: 'utf-8' }).trim().split('\n');

	let stepCounter = 1;
	let templateFound = false;
	let solutionFound = false;

	// Create a folder for each commit
	commitHashes.reverse().forEach((commitInfo, index) => {
		const [commitHash, commitMessage] = commitInfo.split('::');
		const isTemplate = commitMessage.toLowerCase().startsWith('template: ');
		const isSolution = commitMessage.toLowerCase().startsWith('solution: ');

		let stepFolder = path.join(repoPath, stepCounter.toString());
		if (!fs.existsSync(stepFolder)) {
			fs.mkdirSync(stepFolder);
		}

		// Default assumption is output is not a template or solution
		let outputFolder = path.join(stepFolder, "source")

		if (isTemplate) {
			// Check there isn't a template already in queue
			if (templateFound) {
				console.error("A second template was found before a solution.")
				process.exit(1)
			}

			templateFound = true;

			// make step folder
			outputFolder = path.join(stepFolder, "template")
		}

		if (isSolution) {
			// Check that there is a template in queue
			if (!templateFound) {
				console.error("No template was found for this solution.")
				process.exit(1)
			}

			// Check that a solution is not already found.
			if (solutionFound) {
				console.error("A second solution was found before a template.")
				process.exit(1)
			}

			solutionFound = true;
			outputFolder = path.join(stepFolder, "solution")
		}

		fs.mkdirSync(outputFolder);

		// Checkout the commit
		console.log(`Checking out commit: ${commitHash}`);
		execSync(`git -C ${sourcePath} checkout ${commitHash}`);

		// Copy the contents to the commit folder
		execSync(`cp -r ${sourcePath}/* ${outputFolder}`);
		console.log(`Contents of commit ${index + 1} copied to ${outputFolder}`);

		// Reset sanity check and increment step
		// Handle when both template and solution is found,
		// or when there is a step that is neither a template or solution
		if ((templateFound && solutionFound) || (!templateFound && !solutionFound)) {
			let markdownContent = "";
			if (templateFound) {
				markdownContent = templateMarkdown;
			} else {
				markdownContent = sourceMarkdown;
			}
			// Create a Markdown file in the commit folder
			const markdownFilePath = path.join(stepFolder, 'README.md');
			fs.writeFileSync(markdownFilePath, markdownContent);
			stepCounter += 1;
			templateFound = false;
			solutionFound = false;
		}
	});

	// Clean up source folder
	fs.rmSync(sourcePath, { recursive: true, force: true });

	console.log('Process completed.');
});

let templateMarkdown = `
<!-- tabs:start -->

#### **before**

[filename](./template/README.md ':include')

#### **after**

[filename](./solution/README.md ':include')

<!-- tabs:end -->

<!-- slide:break -->

<!-- tabs:start -->

#### **before**

<!-- tabs:start -->

#### **balances.rs**

[filename](./template/src/balances.rs ':include :type=code rust')

#### **main.rs**

[filename](./template/src/main.rs ':include :type=code rust')

<!-- tabs:end -->

#### **after**

<!-- tabs:start -->

#### **balances.rs**

[filename](./solution/src/balances.rs ':include :type=code rust')

#### **main.rs**

[filename](./solution/src/main.rs ':include :type=code rust')

<!-- tabs:end -->

<!-- tabs:end -->
`;

let sourceMarkdown = `
[filename](./source/README.md ':include')

<!-- slide:break -->

<!-- tabs:start -->

#### **balances.rs**

[filename](./source/src/balances.rs ':include :type=code rust')

#### **main.rs**

[filename](./source/src/main.rs ':include :type=code rust')

<!-- tabs:end -->
`;
