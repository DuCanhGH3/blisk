const filteredDependencies = ["svelte", "vite"];

/** @type {import("npm-check-updates").RunOptions} */
module.exports = {
  filter(dependencyName) {
    return !filteredDependencies.includes(dependencyName);
  },
  target(dep) {
    if (dep === "svelte" || dep === "@sveltejs/vite-plugin-svelte") {
      return "@next";
    }
    return "latest";
  },
};
