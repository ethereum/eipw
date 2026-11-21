import {lint, format, Opts} from "../../pkg/eipw_lint_js.js";
import {join} from "path";

describe("eipw_lint_js type checks", () => {
    test("lint function should accept string arrary sources and undefined options", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const expected = `error[preamble-requires-status]: preamble header \`requires\` contains items not stable enough for a \`status\` of \`Last Call\`
  --> tests/eips/eip-1000.md:12:10
   |
12 | requires: 20
   |          ^^^ has a less advanced status
   |
   = help: valid \`status\` values for this proposal are: \`Draft\`, \`Stagnant\`
   = help: see https://ethereum.github.io/eipw/preamble-requires-status/`;
        const res = await lint(sources);
        expect(res[0].formatted).toBe(expected);
    });

    test("lint function should not accept options that are not an instance of Opts", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        //@ts-expect-error
        expect(lint(sources, 1)).rejects.toThrow();
    });

    test("lint function should accept custom options", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options = {
            warn: ["preamble-requires-status"],
            allow: [],
            deny: [],
        };
        const expected = [
            {
                formatted:
                    "warning[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`\n  --> tests/eips/eip-1000.md:12:10\n   |\n12 | requires: 20\n   |          --- has a less advanced status\n   |\n   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`\n   = help: see https://ethereum.github.io/eipw/preamble-requires-status/",
                footer: [
                    {
                        annotation_type: "Help",
                        id: null,
                        label: "valid `status` values for this proposal are: `Draft`, `Stagnant`",
                    },
                    {
                        annotation_type: "Help",
                        id: null,
                        label: "see https://ethereum.github.io/eipw/preamble-requires-status/",
                    },
                ],
                opt: {
                    anonymized_line_numbers: false,
                    color: false,
                },
                slices: [
                    {
                        annotations: [
                            {
                                annotation_type: "Warning",
                                label: "has a less advanced status",
                                range: [9, 12],
                            },
                        ],
                        fold: false,
                        line_start: 12,
                        origin: "tests/eips/eip-1000.md",
                        source: "requires: 20",
                    },
                ],
                title: {
                    annotation_type: "Warning",
                    id: "preamble-requires-status",
                    label: "preamble header `requires` contains items not stable enough for a `status` of `Last Call`",
                },
            },
        ];

        const res = await lint(sources, options);
        expect(res).toEqual(expected);
    });

    test("lint function should not accept wrongly defined type of Opts", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options = {
            warn: [1],
            allow: [],
            deny: [],
        };
        //@ts-expect-error
        expect(lint(sources, options)).rejects.toThrow();
    });

    test("lint function should accept custom default_lints", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options: Opts = {
            default_lints: {
                banana: {
                    kind: "preamble-regex",
                    name: "requires",
                    mode: "includes",
                    pattern: "banana",
                    message: "requires must include banana",
                },
            },
        };
        const expected = [
            {
                formatted:
                    "error[banana]: requires must include banana\n  --> tests/eips/eip-1000.md:12:10\n   |\n12 | requires: 20\n   |          ^^^ required pattern was not matched\n   |\n   = info: the pattern in question: `banana`\n   = help: see https://ethereum.github.io/eipw/banana/",
                footer: [
                    {
                        annotation_type: "Info",
                        id: null,
                        label: "the pattern in question: `banana`",
                    },
                    {
                        annotation_type: "Help",
                        id: null,
                        label: "see https://ethereum.github.io/eipw/banana/",
                    },
                ],
                opt: {
                    anonymized_line_numbers: false,
                    color: false,
                },
                slices: [
                    {
                        annotations: [
                            {
                                annotation_type: "Error",
                                label: "required pattern was not matched",
                                range: [9, 12],
                            },
                        ],
                        fold: false,
                        line_start: 12,
                        origin: "tests/eips/eip-1000.md",
                        source: "requires: 20",
                    },
                ],
                title: {
                    annotation_type: "Error",
                    id: "banana",
                    label: "requires must include banana",
                },
            },
        ];

        const res = await lint(sources, options);
        expect(res).toEqual(expected);
    });

    test("lint function should not accept wrongly defined default_lints", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options: Opts = {
            default_lints: [
                {
                    //@ts-expect-error
                    kind: "set-default-annotation",
                    name: "status",
                    value: "Last Call",
                    annotation_type: "info",
                },
            ],
        };
        expect(lint(sources, options)).rejects.toThrow();
    });

    test("lint function should not accept default_lints with wrongly defined kind", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options: Opts = {
            default_lints: {
                banana: {
                    //@ts-expect-error
                    kind: "test",
                    name: "test",
                    mode: "includes",
                    pattern: "banana",
                    message: "requires must include banana",
                },
            },
        };
        expect(lint(sources, options)).rejects.toThrow();
    });

    test("lint function should not accept default_lints with wrongly defined mode", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options: Opts = {
            default_lints: {
                banana: {
                    kind: "preamble-regex",
                    name: "test",
                    //@ts-expect-error
                    mode: "test",
                    pattern: "banana",
                    message: "requires must include banana",
                },
            },
        };
        expect(lint(sources, options)).rejects.toThrow();
    });

    test("lint function should accept custom default_modifiers", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options: Opts = {
            default_modifiers: [
                {
                    kind: "set-default-annotation",
                    name: "status",
                    value: "Last Call",
                    annotation_type: "info",
                },
            ],
        };
        const expected = [
            {
                formatted:
                    "info[preamble-requires-status]: preamble header `requires` contains items not stable enough for a `status` of `Last Call`\n  --> tests/eips/eip-1000.md:12:10\n   |\n12 | requires: 20\n   |          --- info: has a less advanced status\n   |\n   = help: valid `status` values for this proposal are: `Draft`, `Stagnant`\n   = help: see https://ethereum.github.io/eipw/preamble-requires-status/",
                footer: [
                    {
                        annotation_type: "Help",
                        id: null,
                        label: "valid `status` values for this proposal are: `Draft`, `Stagnant`",
                    },
                    {
                        annotation_type: "Help",
                        id: null,
                        label: "see https://ethereum.github.io/eipw/preamble-requires-status/",
                    },
                ],
                opt: {
                    anonymized_line_numbers: false,
                    color: false,
                },
                slices: [
                    {
                        annotations: [
                            {
                                annotation_type: "Info",
                                label: "has a less advanced status",
                                range: [9, 12],
                            },
                        ],
                        fold: false,
                        line_start: 12,
                        origin: "tests/eips/eip-1000.md",
                        source: "requires: 20",
                    },
                ],
                title: {
                    annotation_type: "Info",
                    id: "preamble-requires-status",
                    label: "preamble header `requires` contains items not stable enough for a `status` of `Last Call`",
                },
            },
        ];
        const res = await lint(sources, options);
        expect(res).toEqual(expected);
    });

    test("lint function should not accept wrongly defined default_modifiers", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options: Opts = {
            default_modifiers: {
                //@ts-expect-error
                banana: {
                    kind: "test",
                    name: "test",
                    mode: "includes",
                    pattern: "banana",
                    message: "requires must include banana",
                },
            },
        };
        expect(lint(sources, options)).rejects.toThrow();
    });

    test("lint function should not accept default_modifiers with wrongly defined kind", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options: Opts = {
            default_modifiers: [
                {
                    //@ts-expect-error
                    kind: "test",
                    name: "status",
                    value: "Last Call",
                    annotation_type: "info",
                },
            ],
        };
        expect(lint(sources, options)).rejects.toThrow();
    });

    test("lint function should not accept default_modifiers with wrongly defined annotation_type", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const options: Opts = {
            default_modifiers: [
                {
                    kind: "set-default-annotation",
                    name: "status",
                    value: "Last Call",
                    //@ts-expect-error
                    annotation_type: "test",
                },
            ],
        };
        expect(lint(sources, options)).rejects.toThrow();
    });

    test("format function should accept a snippet definition", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const expected = `error[preamble-requires-status]: preamble header \`requires\` contains items not stable enough for a \`status\` of \`Last Call\`
  --> tests/eips/eip-1000.md:12:10
   |
12 | requires: 20
   |          ^^^ has a less advanced status
   |
   = help: valid \`status\` values for this proposal are: \`Draft\`, \`Stagnant\`
   = help: see https://ethereum.github.io/eipw/preamble-requires-status/`;
        const res = await lint(sources);
        const actual = format(res[0]);
        expect(actual).toBe(expected);
    });

    test("format function should not accept a snippet definition with a wrong type", async () => {
        const sourcePath = join("tests", "eips", "eip-1000.md");
        const sources = [sourcePath];
        const res = await lint(sources);
        expect(() => {
            //@ts-expect-error
            format(res);
        }).toThrow();
    });
});
