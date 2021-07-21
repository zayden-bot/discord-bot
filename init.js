const fs = require("fs");

module.exports = {
    updateImages: function() {
        const imageConfig = require("./configs/imgConfig.json");
        
        // Update global images with user specific images
        for (obj in imageConfig) {
            try { obj.Global; }
            catch { continue; }

            for (key in imageConfig[obj]) {
                if (key == "Global") { continue; }
                
                for (image of imageConfig[obj][key]) {
                    imageConfig[obj].Global.push(image)
                }
            }
            imageConfig[obj].Global = [...new Set(imageConfig[obj].Global)];
        }

        // Update json file.
        fs.writeFile("./configs/imgConfig.json", JSON.stringify(imageConfig, null, 4), function writeJSON(err) {
            if (err) { return console.log(err); }
        });
    }
}
