module.exports = {
    updateImages: function() {
        const imageConfig: Record<string, Record<string, string[]>> = require("./configs/image_config.json");
        
        // Update global images with user specific images

        for (const categoryName in imageConfig) {
            const category = imageConfig[categoryName]
            let globalImages: Set<string> = new Set();

            for (const user in category) {
                if (user == "Global") {
                    continue;
                }
                const images = category[user]
                images.forEach(image => globalImages.add(image))
            }

            category.Global = [...globalImages];
        }

        // Update json file.
        const fs = require("fs");
        fs.writeFile("./configs/image_config.json", JSON.stringify(imageConfig, null, 4), function writeJSON(err: any) {
            if (err) { return console.log(err); }
        });
    }
}
