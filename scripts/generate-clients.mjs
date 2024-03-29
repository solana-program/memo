#!/usr/bin/env zx
import "zx/globals";
import * as k from "@metaplex-foundation/kinobi";
import { workingDirectory } from "./utils.mjs";

// Instanciate Kinobi.
const kinobi = k.createFromIdls([
  path.join(workingDirectory, "program", "idl.json"),
]);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    addMemo: {
      arguments: {
        memo: {
          type: k.stringTypeNode({ size: k.remainderSizeNode() }),
        },
      },
      remainingAccounts: [
        k.instructionRemainingAccountsNode(k.argumentValueNode("signers"), {
          isOptional: true,
          isSigner: true,
        }),
      ],
    },
  })
);

// Render JavaScript.
const jsClient = path.join(__dirname, "..", "clients", "js");
kinobi.accept(
  k.renderJavaScriptExperimentalVisitor(
    path.join(jsClient, "src", "generated"),
    { prettier: require(path.join(jsClient, ".prettierrc.json")) }
  )
);

// Render Rust.
const rustClient = path.join(__dirname, "..", "clients", "rust");
kinobi.accept(
  k.renderRustVisitor(path.join(rustClient, "src", "generated"), {
    formatCode: true,
    crateFolder: rustClient,
  })
);
