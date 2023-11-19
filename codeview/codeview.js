require.config({ paths: { vs: 'https://unpkg.com/monaco-editor@0.44.0/min/vs' } });

const editorElement = 'editor';

function loadDiffEditor(original, modified) {
	require(['vs/editor/editor.main'], function () {
		var diffEditor = monaco.editor.createDiffEditor(document.getElementById(editorElement));

		diffEditor.setModel({
			original: monaco.editor.createModel(original, 'javascript'),
			modified: monaco.editor.createModel(modified, 'javascript')
		});
	});
}

function oldLoadDiffEditor() {
	require(['vs/editor/editor.main'], function () {
		var diffEditor = monaco.editor.createDiffEditor(document.getElementById(editorElement));

		Promise.all([xhr('original.txt'), xhr('modified.txt')]).then(function (r) {
			var originalTxt = r[0].responseText;
			var modifiedTxt = r[1].responseText;

			diffEditor.setModel({
				original: monaco.editor.createModel(originalTxt, 'javascript'),
				modified: monaco.editor.createModel(modifiedTxt, 'javascript')
			});
		});
	});
}

function xhr(url) {
	var req = null;
	return new Promise(
		function (c, e) {
			req = new XMLHttpRequest();
			req.onreadystatechange = function () {
				if (req._canceled) {
					return;
				}

				if (req.readyState === 4) {
					if ((req.status >= 200 && req.status < 300) || req.status === 1223) {
						c(req);
					} else {
						e(req);
					}
					req.onreadystatechange = function () { };
				}
			};

			req.open('GET', url, true);
			req.responseType = '';

			req.send(null);
		},
		function () {
			req._canceled = true;
			req.abort();
		}
	);
}

window.$docsify.plugins.push(
	function (hook, vm) {
		console.log("Docsify Window")
	}
);
