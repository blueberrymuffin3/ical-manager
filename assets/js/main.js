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

function setInputHide(name, hide) {
  let section = document.getElementById(name);

  if (hide) {
    section.classList.add("hide");
  } else {
    section.classList.remove("hide");
  }
}

function updateFormForm() {
  let type = document.querySelector("#form-form #type").value;

  setInputHide("section-link", type != "link");
  setInputHide("section-upload", type != "upload");
}

if (document.querySelector("#form-form")) {
  updateFormForm();
}
