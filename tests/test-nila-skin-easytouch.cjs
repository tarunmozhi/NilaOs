"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const html = fs.readFileSync(
  path.join(__dirname, "../apps/systemui/nila-skin/index.html"),
  "utf8",
);
const scripts = [...html.matchAll(/<script>([\s\S]*?)<\/script>/g)];
assert.equal(scripts.length, 1, "Nila Skin should have one inline script");

const shortcutIds = [
  "phone", "messages", "browser", "camera", "files",
  "network", "bluetooth", "display", "security", "vault",
];

function createClassList() {
  const values = new Set();
  return {
    add(value) { values.add(value); },
    remove(value) { values.delete(value); },
    contains(value) { return values.has(value); },
    toggle(value, force) {
      const next = force === undefined ? !values.has(value) : force;
      if (next) values.add(value);
      else values.delete(value);
      return next;
    },
  };
}

function createEnvironment(width, height, storage) {
  const elements = new Map();
  const links = new Map();
  const options = shortcutIds.map((id) => ({
    dataset: { easytouchOption: id },
    checked: true,
    listeners: {},
    addEventListener(type, listener) { this.listeners[type] = listener; },
  }));

  function getLink(id) {
    if (!links.has(id)) links.set(id, { hidden: false });
    return links.get(id);
  }

  function getElement(id) {
    if (elements.has(id)) return elements.get(id);
    const element = {
      id,
      hidden: false,
      checked: id === "easytouch-enabled",
      style: {},
      classList: createClassList(),
      listeners: {},
      attributes: {},
      textContent: "",
      scrollTop: 0,
      addEventListener(type, listener) { this.listeners[type] = listener; },
      setAttribute(name, value) { this.attributes[name] = value; },
      setPointerCapture() {},
      querySelector(selector) {
        if (id === "easytouch-app-group") {
          return ["phone", "messages", "browser", "camera", "files"]
            .map(getLink).find((link) => !link.hidden) || null;
        }
        if (id === "easytouch-settings-group") {
          return ["network", "bluetooth", "display", "security", "vault"]
            .map(getLink).find((link) => !link.hidden) || null;
        }
        return { hidden: false };
      },
      getBoundingClientRect() {
        const widget = getElement("easytouch-widget");
        if (id === "easytouch-trigger") {
          if (widget.hidden) return { left: 0, top: 0, right: 0, bottom: 0 };
          const left = widget.style.left ? Number.parseFloat(widget.style.left) : width - 36;
          const top = widget.style.top ? Number.parseFloat(widget.style.top) : height - 78;
          return { left, top, right: left + 54, bottom: top + 54 };
        }
        return { left: 0, top: 0, right: 200, bottom: 300 };
      },
    };
    Object.defineProperty(element, "offsetWidth", {
      get() {
        if (id !== "easytouch-trigger") return 200;
        return getElement("easytouch-widget").hidden ? 0 : 54;
      },
    });
    Object.defineProperty(element, "offsetHeight", {
      get() {
        if (id !== "easytouch-trigger") return 300;
        return getElement("easytouch-widget").hidden ? 0 : 54;
      },
    });
    elements.set(id, element);
    return element;
  }

  const document = {
    documentElement: { style: { setProperty() {} } },
    getElementById: getElement,
    addEventListener() {},
    querySelectorAll(selector) {
      return selector === "[data-easytouch-option]" ? options : [];
    },
    querySelector(selector) {
      const linkMatch = selector.match(/^\[data-easytouch-link="([^"]+)"\]$/);
      if (linkMatch) return getLink(linkMatch[1]);
      if (selector === ".device") return getElement(".device");
      if (selector === "[data-easytouch-link]:not([hidden])") {
        return [...links.values()].find((link) => !link.hidden) || null;
      }
      return null;
    },
  };
  const window = {
    innerWidth: width,
    innerHeight: height,
    localStorage: storage,
    listeners: {},
    addEventListener(type, listener) { this.listeners[type] = listener; },
    scrollTo() {},
  };
  vm.runInNewContext(scripts[0][1], {
    document,
    window,
    setTimeout: () => 1,
    clearTimeout() {},
  });
  return { document, window, elements, options, links };
}

function createStorage() {
  const values = new Map();
  return {
    values,
    getItem(key) { return values.get(key) ?? null; },
    setItem(key, value) { values.set(key, value); },
  };
}

test("Easy Touch is enabled by default and peeks from the screen edge", () => {
  assert.match(html, /\.easytouch-widget\{position:fixed;right:-18px;bottom:24px/);
  assert.match(html, /@media\(max-width:760px\)\{\.easytouch-widget\{right:-18px;bottom:16px/);
  const env = createEnvironment(400, 800, createStorage());
  assert.equal(env.elements.get("easytouch-enabled").checked, true);
  assert.equal(env.elements.get("easytouch-widget").hidden, false);
});

test("toggle, shortcuts, and moved position persist across reload and resize", () => {
  const storage = createStorage();
  const first = createEnvironment(400, 800, storage);
  const widget = first.elements.get("easytouch-widget");
  const trigger = first.elements.get("easytouch-trigger");

  trigger.listeners.pointerdown({
    pointerType: "touch", pointerId: 1, clientX: 380, clientY: 740,
  });
  trigger.listeners.pointermove({
    pointerId: 1, clientX: 340, clientY: 300, preventDefault() {},
  });
  trigger.listeners.pointerup({ pointerId: 1 });

  const afterDrag = JSON.parse(storage.getItem("nila-os-easytouch-v1"));
  assert.ok(widget.classList.contains("is-moved"));
  assert.ok(afterDrag.position);

  const phone = first.options.find((option) => option.dataset.easytouchOption === "phone");
  phone.checked = false;
  phone.listeners.change();
  assert.equal(first.links.get("phone").hidden, true);

  const toggle = first.elements.get("easytouch-enabled");
  toggle.checked = false;
  toggle.listeners.change({ target: toggle });
  const saved = JSON.parse(storage.getItem("nila-os-easytouch-v1"));
  assert.equal(saved.enabled, false);
  assert.equal(saved.shortcuts.phone, false);
  assert.deepEqual(saved.position, afterDrag.position);

  const second = createEnvironment(800, 600, storage);
  assert.equal(second.elements.get("easytouch-enabled").checked, false);
  assert.equal(second.elements.get("easytouch-widget").hidden, true);
  assert.equal(second.options.find((option) => option.dataset.easytouchOption === "phone").checked, false);
  assert.equal(second.elements.get("easytouch-widget").style.left, "648px");
  assert.equal(second.elements.get("easytouch-widget").style.top, "211.5px");

  second.window.innerWidth = 500;
  second.window.innerHeight = 400;
  second.window.listeners.resize();
  assert.equal(second.elements.get("easytouch-widget").hidden, true);
  assert.equal(second.elements.get("easytouch-widget").style.left, "405px");
  assert.equal(second.elements.get("easytouch-widget").style.top, "141px");

  const reenable = second.elements.get("easytouch-enabled");
  reenable.checked = true;
  reenable.listeners.change({ target: reenable });
  assert.equal(second.elements.get("easytouch-widget").hidden, false);
});

test("keyboard movement may partly tuck the button at either edge", () => {
  const env = createEnvironment(400, 800, createStorage());
  const trigger = env.elements.get("easytouch-trigger");
  for (let i = 0; i < 30; i += 1) {
    trigger.listeners.keydown({ key: "ArrowLeft", shiftKey: false, preventDefault() {} });
  }
  assert.equal(env.elements.get("easytouch-widget").style.left, "-24px");
  const saved = JSON.parse(env.window.localStorage.getItem("nila-os-easytouch-v1"));
  assert.ok(saved.position.x < 0);
});
