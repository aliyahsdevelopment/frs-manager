import path from "path";
import { CommandHandler, DataRecord } from "../utils/types";
import { existsSync, readFileSync, writeFileSync } from "fs";
import pc from "picocolors";
import z from "zod";

const ConfigStructure = z.object({
  name: z.string(),
  tasks: z.array(z.string()).optional(),
  ignore: z.array(z.string()).optional()
});

const handler: CommandHandler = args => {
  const isVerbose = args.v == true || args.verbose == true;
  const dataFilePath = path.join(path.dirname(__dirname), "data.json");
  let data: Record<string, DataRecord> = {};

  if (existsSync(dataFilePath) == false) {
    writeFileSync(dataFilePath, "{}");
  }

  try {
    const rawData = readFileSync(dataFilePath, "utf8");
    data = JSON.parse(rawData);
  } catch (err) {
    console.warn(
      pc.redBright("Your data.json file for zBundler seems to be invalid")
    );
    console.info(err);
  }

  const verboseLog = (msg: string): void => {
    if (isVerbose == true) {
      console.log(msg);
    }
  };

  void data;
  void ConfigStructure;
  void verboseLog;
};

export default handler;
