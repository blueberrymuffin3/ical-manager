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
 * @param {string} formId The id of the form with the groups to hide/show
 * @param {string} id The id of the select element
 */
function updateSelectGroups(formId, id) {
  let form = document.getElementById(formId);
  if (!form) return;

  let value = form.querySelector(`#${id}`).value;

  for (group of form.querySelectorAll(`[data-show-for-id=${id}]`)) {
    if (group.dataset.showForValue == value) {
      group.classList.remove("hide");
    } else {
      group.classList.add("hide");
    }
  }
}

function updateAllSelectGroups() {
  updateSelectGroups("feed-form", "source-type");
}

document.addEventListener("htmx:afterSwap", updateAllSelectGroups)


updateAllSelectGroups();
