console.log("dashboard.js loaded");

function analyzeStock() {
  const ticker = document.getElementById("ticker").value.trim().toUpperCase();

  if (!ticker) return;

  document.getElementById("loading").classList.remove("hidden");
  document.getElementById("results").classList.add("hidden");

  fetch("http://localhost:3000/analyze/" + ticker)
    .then(function (response) {
      return response.json();
    })
    .then(function (data) {
      if (data.error) {
        alert("Error: " + data.error);
        return;
      }

      document.getElementById("stock-symbol").textContent = data.symbol;
      document.getElementById("stock-price").textContent =
        "Price: $" + data.price.toFixed(2);
      document.getElementById("stock-analysis").innerHTML = data.analysis
        .replace(/## (.*)/g, "<h2>$1</h2>")
        .replace(/### (.*)/g, "<h3>$1</h3>")
        .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
        .replace(/- (.*)/g, "<li>$1</li>")
        .replace(/\n/g, "<br>");
      document.getElementById("results").classList.remove("hidden");
    })
    .catch(function (err) {
      alert("Something went wrong: " + err.message);
    })
    .finally(function () {
      document.getElementById("loading").classList.add("hidden");
    });
}

document.getElementById("ticker").addEventListener("keypress", function (e) {
  if (e.key === "Enter") analyzeStock();
});
