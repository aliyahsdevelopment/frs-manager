import path from "path";
import { CommandHandler, Config, DataRecord } from "../utils/types";
import { existsSync, readFileSync, rmSync, writeFileSync } from "fs";
import pc from "picocolors";
import z from "zod";
import { execSync } from "child_process";
import { globalBundlerIgnore } from "../utils/data";

const ConfigStructure = z.object({
  name: z.string(),
  tasks: z.array(z.string()).optional(),
  ignore: z.array(z.string()).optional()
});

function getFxManifestVersion(text: string): string | null {
  text = text.replace("fx_version", "");
  const idx = text.indexOf("version");

  if (idx !== -1) {
    const lines = text.substring(idx).split(/\r?\n|\r|\n/g);

    if (lines.length >= 1) {
      const line = lines[0];
      const splits = line.split(line.includes("'") == true ? "'" : '"');

      if (splits.length >= 2) {
        return splits[1];
      }
    }
  }

  return null;
}

function fxManifestContainsDevUiPage(text: string): boolean {
  const lines = text
    .split(/\r?\n|\r|\n/g)
    .filter((line: string) => line.includes("ui_page"));

  for (const line of lines) {
    if (line.includes("--") == false && line.includes("localhost") == true) {
      return true;
    }
  }

  return false;
}

const handler: CommandHandler = async args => {
  const isVerbose = args.v == true || args.verbose == true;
  const dataFilePath = path.join(path.dirname(__dirname), "data.json");
  let data: Record<string, DataRecord> = {};
  let config: Config | null = null;
  const cwd = process.cwd();

  const verboseLog = (msg: unknown): void => {
    if (isVerbose == true) {
      console.log(msg);
    }
  };

  if (existsSync(dataFilePath) == false) {
    writeFileSync(dataFilePath, "{}");
  }

  console.log("")

  try {
    const rawData = readFileSync(dataFilePath, "utf8");
    data = JSON.parse(rawData);
  } catch (err) {
    console.warn(
      pc.redBright("Your data.json file for zBundler seems to be invalid")
    );
    verboseLog(err);
  }

  if (typeof data !== "object" || Array.isArray(data) === true) {
    data = {};
  }

  try {
    const importedConfig = await import(path.join(cwd, "bundler.config.mjs"));

    config = importedConfig.default;
  } catch (err) {
    try {
      verboseLog(err);

      const importedConfig = await import(path.join(cwd, "bundler.config.js"));

      config = importedConfig.default;
    } catch (e2) {
      console.warn(pc.redBright("Could not find the bundler config file"));
      verboseLog(e2);
    }
  }

  if (config !== null && ConfigStructure.safeParse(config).success === true) {
    const fxmanifestData = readFileSync(
      path.join(cwd, "fxmanifest.lua")
    ).toString();

    const version = getFxManifestVersion(fxmanifestData);

    if (version !== null) {
      if (data[cwd] !== undefined) {
        if (data[cwd].version == version) {
          console.warn(
            pc.yellowBright("Dont forget to update the resource version")
          );
        }

        data[cwd].version = version;
      } else {
        data[cwd] = {
          version: version
        };
      }
    }

    if (fxManifestContainsDevUiPage(fxmanifestData) == true) {
      console.warn(
        pc.redBright(
          "Dont forget to remove/comment out the localhost ui_page!!"
        )
      );
    }

    writeFileSync(dataFilePath, JSON.stringify(data));

    if (config.tasks) {
      for (let i = 0; i < config.tasks.length; i++) {
        execSync(config.tasks[i], isVerbose ? { stdio: [0, 1, 2] } : undefined);
      }
    }

    let zipCmd = `7z a ${config.name}.zip`;

    if (config.ignore) {
      config.ignore = [...config.ignore, ...globalBundlerIgnore];

      for (let i = 0; i < config.ignore.length; i++) {
        zipCmd += ` -x!${config.ignore[i]}`;
      }
    }

    const zipPath = path.join(cwd, `${config.name}.zip`);
    if (existsSync(zipPath)) {
      rmSync(zipPath);
    }

    execSync(zipCmd, isVerbose ? { stdio: [0, 1, 2] } : undefined);

    console.log(pc.greenBright("Successfully bundled files"));
  }
};

export default handler;
