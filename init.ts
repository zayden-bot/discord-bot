module.exports = {
    updateImages: function() {
        const imageConfig: { [key: string]: { [key: string]: string[] } } = require("./configs/imgConfig.json");
        
        // Update global images with user specific images
        for (const [catagory, obj] of Object.entries(imageConfig)) {
        
            let globalImages: Set<string> = new Set();
            for (const [user, images] of Object.entries(obj)) { 
                if (user === "Global") { continue; }
                
                images.forEach(image => globalImages.add(image))
                obj.Global = [...globalImages];
            }
        }

        // Update json file.
        const fs = require("fs");
        fs.writeFile("./configs/imgConfig.json", JSON.stringify(imageConfig, null, 4), function writeJSON(err: any) {
            if (err) { return console.log(err); }
        });
    }
}
