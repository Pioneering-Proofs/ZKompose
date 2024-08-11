import "abitype";

declare module "wagmi/node_modules/abitype" {
  export interface Config {
    BytesType: Uint8Array | string;
  }
}
