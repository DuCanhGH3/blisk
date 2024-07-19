const filteredDependencies = ["svelte", "vite"];

/** @type {import("npm-check-updates").RunOptions} */
module.exports = {
  filter(dependencyName) {
    return !filteredDependencies.includes(dependencyName);
  },
  target(dependencyName) {
    if (dependencyName === "svelte") {
      return "@next";
    }
    if (dependencyName === "typescript-eslint") {
      return "@rc-v8";
    }
    return "latest";
  },
};
