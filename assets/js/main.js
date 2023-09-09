/**
 * @param {HTMLButtonElement} target
 */
function copyFeedLink(target) {
  for (const shown of document.querySelectorAll(".copy-source-shown")) {
    shown.classList.remove("copy-source-shown");
  }

  /** @type {HTMLInputElement} */
  let input = target.parentElement.querySelector("input.copy-source");
  input.classList.add("copy-source-shown");
  input.value = new URL(input.dataset.partialCopyUri, window.location);
  input.select();
  navigator.clipboard.writeText(input.value);
}

/**
 * @this {HTMLSelectElement}
 */
function updateSelectGroups() {
  const form = document.getElementById(this.dataset.triggerShowHideForm);
  if (!form) return;

  const value = form.querySelector(`#${this.id}`).value;

  for (group of form.querySelectorAll(`[data-show-for-id=${this.id}]`)) {
    if (group.dataset.showForValue == value) {
      group.classList.remove("hide");
    } else {
      group.classList.add("hide");
    }
  }
}

function updateAllSelectGroups() {
  for (const select of document.querySelectorAll("[data-trigger-show-hide-form]")) {
    select.removeEventListener("change", updateSelectGroups);
    select.addEventListener("change", updateSelectGroups);
    updateSelectGroups.call(select)
  }
}

document.addEventListener("htmx:afterSwap", updateAllSelectGroups);
updateAllSelectGroups();
