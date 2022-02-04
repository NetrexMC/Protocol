// this script uses deno.
// to run it, use:
// deno run --allow-read --allow-write --allow-net scripts/generate_versions.ts
let versions = await readJson("resources/versions.json");
let list: [string, any][] = Object.entries(versions);

// @ts-ignore
let version_list: [string, any][] = [];
list.forEach((v, i) => {
    if (version_list.findIndex((t) => t[1] === v[1]) === -1) {
        version_list.push(v);
    } else {
        // we might need to replace this version
        // check whether it's a major version
        if ((/^[0-9]\.[0-9]+\.0$/).test(v[0])) {
            // get the index of the version
            let index = version_list.findIndex((t) => t[1] === v[1]);
            // replace the version
            version_list[index] = v;
        }
    }
})
let final_list = version_list.map((v: [string, any]) => {
    let literal = v[0].replace(/\./g, "_");
    return `V${literal} = ${v[1]},`;
});

const MINOR = list.find((v) => (/^[0-9]\.[0-9]+\.[0-9]+$/).test(v[0])) || ["none", 0];
const MAJOR = list.find((v) => (/^[0-9]\.[0-9]+\.0$/).test(v[0])) || ["none", 0];

console.log(`Using ${list[0][0]} as the current patch\nUsing ${MINOR[0]} as the minor version\nUsing ${MAJOR[0]} as the major version`);

let new_file = `// This file has been auto-generated by scripts/generate_versions.ts
// DO NOT EDIT THIS FILE
// To add versions, add a version to the resources/versions.json file!

/// The current minecraft patch version (1.10.2.x)
pub const CURRENT_PATCH: u32 = ${list[0]?.[1] || 0};
/// The current minecraft minor version (1.10.x)
pub const CURRENT_MINOR: u32 = ${MINOR[1] || 0};
/// The current minecraft major version (1.x.0)
pub const CURRENT_MAJOR: u32 = ${MAJOR[1] || 0};

/// This is a helper enum to get a version number from a version string.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum Versions {
    ${final_list.join("\n    ")}
}

pub fn version_within_current_patch(version: u32) -> bool {
    version >= CURRENT_MAJOR && version <= CURRENT_PATCH
}

pub fn version_within_current_minor(version: u32) -> bool {
    version >= CURRENT_MAJOR && version <= CURRENT_MINOR
}`;
await Deno.writeTextFile("src/mcpe/version.rs", new_file);

async function readJson(file: string): Promise<any> {
    return JSON.parse(await Deno.readTextFile(file));
}