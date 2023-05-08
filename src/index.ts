import fs from "fs"

const fileName = process.argv[2];

if (fileName) {


    fs.readFileSync(fileName).toString().split("\n")
        .forEach(line => {
            const print = parseInt(line)
            if (isNaN(print)) {
                console.log("Linenot a number")
            }
            else { console.log(print) }
        }
        );

}