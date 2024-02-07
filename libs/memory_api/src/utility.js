/**
 * @type {import("@keywich/api").UtilityRPC}
 */
const module = {
  save_file(fileData, path) {
    return Promise.resolve({
      success: true
    });
  }
}

export default module;