import * as vscode from "vscode";
import * as decondenser from "decondenser";

export async function activate(ctx: vscode.ExtensionContext) {
    // Can't use a top-level await unfortunately. We'll need to wait for the
    // native ESM support in VSCode extensions:
    // https://github.com/microsoft/vscode/issues/130367
    await decondenser.$init;

    function addCommand(commandName: string, action: () => void) {
        const command = vscode.commands.registerCommand(commandName, action);
        ctx.subscriptions.push(command);
    }

    addCommand("decondenser.decondense", () => {
        decondense(false);
    });
    addCommand("decondenser.unescape.decondense", () => {
        decondense(true);
    });
}

function decondense(shouldUnescape: boolean) {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        return;
    }
    const { document } = editor;

    const selection = editor.selection.isEmpty
        ? fullDocumentSelection(document)
        : editor.selection;

    let input = editor.document.getText(selection);

    if (shouldUnescape) {
        input = decondenser.unescape(input);
    }

    const output = decondenser.decondense(input, " ".repeat(getIndent()));

    editor.edit((edit) => {
        edit.replace(selection, output);
    });
}

function fullDocumentSelection(
    document: vscode.TextDocument,
): vscode.Selection {
    const firstLine = document.lineAt(0);
    const lastLine = document.lineAt(document.lineCount - 1);
    return new vscode.Selection(
        firstLine.rangeIncludingLineBreak.start,
        lastLine.rangeIncludingLineBreak.end,
    );
}

function getIndent(): number {
    const indent = vscode.workspace
        .getConfiguration("decondenser")
        .get("indentationSize");

    if (typeof indent !== "number" || indent < 0) {
        return 2;
    }

    return indent;
}
