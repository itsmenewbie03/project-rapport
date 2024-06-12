function generateHexId() {
  return window.crypto.randomUUID();
}

export { generateHexId as gen_uuid };
