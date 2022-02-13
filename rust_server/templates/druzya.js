function requestStep() {
    var req = new XMLHttpRequest();
    req.open("POST", "/step");

    req.onreadystatechange = function(ev) {
        if (this.readyState == 4) {
            if (this.status == 200) {
                document.getElementById("steps").innerHTML += "<li>" + this.responseText + "</li>";
            } else {
                alert("Step request failed.");
            }
        }
    }

    req.send();
}