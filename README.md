## Rust by example

> Harshith's version

Me going through https://doc.rust-lang.org/rust-by-example/ and documenting my solutions for any questions

---

Deleting the binaries after each run using Code Runner config

If you want to delete the output files after each run, you can modify the Code Runner settings in VS Code.

Here’s how you can set up Code Runner to automatically clean up the binary files after running:
	1.	Open VS Code settings (click the gear icon in the lower-left corner, then “Settings”).
	2.	Search for “Code Runner: Executor Map” in the settings search bar.
	3.	Click “Edit in settings.json” to open the configuration file.

In the settings.json file, you’ll need to modify the executor map for Rust to run cargo run and then delete the binaries. Add the following configuration:

```json
{
    "files.associations": {
        "*.py": "python"
    },
    "files.autoSave": "afterDelay",
    "editor.largeFileOptimizations": false,
    "code-runner.executorMap": {
        .
        .
        .
        "r": "Rscript",
        "applescript": "osascript",
        "clojure": "lein exec",
        "haxe": "haxe --cwd $dirWithoutTrailingSlash --run $fileNameWithoutExt",
        "rust": "cd $dir && rustc $fileName && $dir$fileNameWithoutExt && rm -f $dir$fileNameWithoutExt",
        "racket": "racket",
        .
        .
        .
    }
}
```

Notice that
```
rm -f $dir$fileNameWithoutExt
```
is added addtionally.

---