var blusas = [],
    pants = [],
    indexBlusa = indexPant = 0,
    bodyBoceto = bodyTop = bodyBottom = undefined,
    bodyHuman;

var updateUI = () => {
    if(bodyTop != undefined)
        bodyTop.style.backgroundImage = `url(${blusas[indexBlusa]})`;
    if(bodyBottom != undefined)
        bodyBottom.style.backgroundImage = `url(${pants[indexPant]})`;
}

document.addEventListener("DOMContentLoaded", function() {
    for (var i = 1; i <= 17; i++){
        if(i <= 6)
            blusas.push(`./assets/Modelos/blusa-${i}.png`);
        pants.push(`./assets/Modelos/falda-${i}.png`);
    }
    pants.push("./assets/Modelos/pantalon-1.png");
    pants.push("./assets/Modelos/pantalon-2.png");

    bodyHuman = document.getElementById("base-human");
    bodyTop = document.getElementById("body-blusa");
    bodyBottom = document.getElementById("body-falda");
    
    document.getElementById("btn-blusa-left").addEventListener("click", () => {
        indexBlusa = (indexBlusa - 1 < 0) ? blusas.length - 1 : indexBlusa--;
        updateUI();
    })
    document.getElementById("btn-blusa-right").addEventListener("click", () => {
        indexBlusa = (indexBlusa++ >= blusas.length-1) ? 0 : indexBlusa++;
        updateUI();
    })
    document.getElementById("btn-pant-left").addEventListener("click", () => {
        indexPant = (indexPant - 1 < 0) ? pants.length - 1 : indexPant--;
        updateUI();
    })
    document.getElementById("btn-pant-right").addEventListener("click", () => {
        indexPant = (indexPant++ >= pants.length-1) ? 0 : indexPant++;
        updateUI();
    })
    document.getElementById("btn-screenshot").addEventListener("click", () => {
        domtoimage.toBlob(bodyHuman)
            .then(function (blob) {
                var a = document.createElement("a");
                document.body.appendChild(a);
                a.style = "display: none";
                url = window.URL.createObjectURL(blob);
                a.href = url;
                a.download = fileName;
                a.click();
                window.URL.revokeObjectURL(url);
            });
    })
});
