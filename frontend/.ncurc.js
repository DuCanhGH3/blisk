const filteredDependencies = [];

/** @type {import("npm-check-updates").RunOptions} */
module.exports = {
  filter(dependencyName) {
    return !filteredDependencies.includes(dependencyName);
  },
  target(dependencyName) {
    if (dependencyName.includes("serwist")) {
      return "@preview";
    }
    return "latest";
  },
};
