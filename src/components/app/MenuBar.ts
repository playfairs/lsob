const menuGroups = [
  {
    label: "File",
    items: ["New", "Open", "Open Recent", "Save", "Save As", "Export", "Quit"],
  },
  {
    label: "Edit",
    items: ["Undo", "Redo", "Cut", "Copy", "Paste", "Preferences"],
  },
  {
    label: "Image",
    items: ["Mode", "Canvas Size", "Scale Image", "Crop", "Transform"],
  },
  {
    label: "Layer",
    items: ["New Layer", "Duplicate Layer", "Merge", "Mask", "Effects"],
  },
  {
    label: "Colors",
    items: [
      "Brightness / Contrast",
      "Levels",
      "Curves",
      "Hue / Saturation",
      "Desaturate",
    ],
  },
  {
    label: "Filters",
    items: [
      "Blur",
      "Enhance",
      "Distorts",
      "Light & Shadow",
      "Noise",
      "Edge Detect",
      "Generic",
      "Combine",
      "Artistic",
      "Decor",
      "Map",
      "Render",
      "Web",
      "Animation",
    ],
  },
  {
    label: "View",
    items: ["Zoom", "Show Grid", "Show Guides", "Fullscreen", "Panels"],
  },
];

export function MenuBar() {
  return `
    <header class="menu-bar">
      <div class="brand-block">
        <span class="brand-badge">lsob</span>
        <span class="brand-subtitle">IMAGE EDITOR</span>
      </div>
      <nav class="menu-groups" aria-label="Main menu">
        ${menuGroups
          .map(
            (group) => `
              <div class="menu-group" data-menu-group="${group.label}">
                <button class="menu-button" type="button" aria-expanded="false" data-menu-button="${group.label}">
                  ${group.label}
                </button>
                <div class="menu-popup" role="menu" aria-label="${group.label} menu">
                  ${group.items
                    .map(
                      (item) =>
                        `<button class="menu-item" type="button" data-menu-item="${item}">${item}</button>`,
                    )
                    .join("")}
                </div>
              </div>
            `,
          )
          .join("")}
      </nav>
    </header>
  `;
}
