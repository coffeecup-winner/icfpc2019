"use strict";
let fs = require('fs');

if (process.argv.length != 4) {
    process.stdout.write("Usage: node validator/main.js DESC SOL");
    process.exit();
}

const DESCRIPTION_FILE = process.argv[2];
const SOLUTION_FILE = process.argv[3];

// The validator will use `window` for callback subscription and delaying execution
global.window = {
    document: {
        // Elements that we'll capture by id
        elements: {},
        getElementById: function(id) {
            // We only care about elements the validator will put callbacks on
            // NOTE: This code expects this method to be called once per id, it's currently true
            this.elements[id] = { appendChild: function(_) { return null; } };
            return this.elements[id];
        },
        createElement: function(_) {
            return { appendChild: function(_) { return null; } };
        }
    },
    setTimeout: function(func, _timeout) {
        // We don't need delay
        func();
    }
}

// The validator uses this in an `instanceof` check, this is a file input object
global.Blob = function(path) {
    this.path = path;
}

// The validator uses this to read a local file, implementing it with Node.js APIs instead
global.FileReader = function() {
    return {
        onloadend: null, // The validator will set this callback before calling `readAsText`
        result: null, // The validator will read the result from this field
        readAsText: function(blob) {
            this.result = fs.readFileSync(blob.path).toString();
            this.onloadend(null);
        }
    };
}

// Include the validator script into this scope, source: https://icfpcontest2019.github.io/solution_checker
eval.apply(global, [fs.readFileSync(__dirname + '/validator.js').toString()]);
// Call the top-level validator method to subscribe the callbacks
validate();

let e = window.document.elements;
e.submit_task.files = [new Blob(DESCRIPTION_FILE)];
e.submit_task.onchange();
e.submit_solution.files = [new Blob(SOLUTION_FILE)];
e.submit_solution.onchange();
e.execute_solution.onclick();

let output = e.output.textContent
let result = {}
let success = output.match(/Success! \n?Your solution took (\d+) time units\./);
if (success) {
    result.success = true;
    result.time = parseInt(success[1]);
} else {
    result.success = false;
    let failure = output.match(/.*[F|f]ailed.*/);
    if (failure) {
        result.error = failure[0];
    } else {
        result.error = "Unexpected validator output: '" + output + "'";
    }
}
process.stdout.write(JSON.stringify(result));
