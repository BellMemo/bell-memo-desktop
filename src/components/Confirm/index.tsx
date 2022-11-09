import { createRoot } from "react-dom/client";
import { Confirm, ConfirmProps } from "./Confirm";

type ConfirmFunctionProps = Omit<ConfirmProps, "type" | "defaultVisible">;

const confirm = {
  success(props: ConfirmFunctionProps): Promise<boolean> {
    const ele = document.createElement("div");
    return new Promise((s) => {
      const JSXdom = (
        <Confirm
          {...props}
          type="success"
          onOk={() => s(true)}
          onCancel={() => s(false)}
          defaultVisible
        ></Confirm>
      );
      createRoot(ele).render(JSXdom);
    });
  },
  error(props: ConfirmFunctionProps): Promise<boolean> {
    const ele = document.createElement("div");
    return new Promise((s) => {
      const JSXdom = (
        <Confirm
          {...props}
          type="danger"
          onOk={() => s(true)}
          onCancel={() => s(false)}
          defaultVisible
        ></Confirm>
      );
      createRoot(ele).render(JSXdom);
    });
  },
  warning(props: ConfirmFunctionProps): Promise<boolean> {
    const ele = document.createElement("div");
    return new Promise((s) => {
      const JSXdom = (
        <Confirm
          {...props}
          type="warning"
          onOk={() => s(true)}
          onCancel={() => s(false)}
          defaultVisible
        ></Confirm>
      );
      createRoot(ele).render(JSXdom);
    });
  },
  info(props: ConfirmFunctionProps): Promise<boolean> {
    const ele = document.createElement("div");
    return new Promise((s) => {
      const JSXdom = (
        <Confirm
          {...props}
          type="default"
          onOk={() => s(true)}
          onCancel={() => s(false)}
          defaultVisible
        ></Confirm>
      );
      createRoot(ele).render(JSXdom);
    });
  },
};

export { confirm, Confirm };
