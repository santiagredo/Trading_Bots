import { create } from "zustand"

export type Environment = "dev" | "prod"

interface EnvironmentStore {
  environment: Environment
  setEnvironment: (env: Environment) => void
}

export const useEnvironment = create<EnvironmentStore>((set) => ({
  environment: "dev",
  setEnvironment: (environment) => set({ environment }),
}))
