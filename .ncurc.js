/** @type {import("npm-check-updates").RunOptions} */
module.exports = {
  filter(dependencyName) {
    return dependencyName !== "eslint";
  },
  target(dependencyName) {
    if (dependencyName === "svelte") {
      return "@next";
    }
    return "latest";
  },
};
