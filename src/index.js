document.addEventListener("DOMContentLoaded", () => {
  let i = 0;
  setInterval(() => {
    i += 1;
    document.body.textContent = `Check out ${i}`;
  }, 1000);
});
