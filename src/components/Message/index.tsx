import { createRoot } from "react-dom/client";
import { Message } from "./Message";

const DEFAULT_DURATION = 1500;

const message = {
  dom: null,
  success({ content, duration = DEFAULT_DURATION }) {
    this.dom = document.createElement("div");
    const JSXdom = (
      <Message content={content} duration={duration} type="success"></Message>
    );
    createRoot(this.dom).render(JSXdom);
    document.body.appendChild(this.dom);
  },
  error({ content, duration = DEFAULT_DURATION }) {
    this.dom = document.createElement("div");
    const JSXdom = (
      <Message content={content} duration={duration} type="error"></Message>
    );
    createRoot(this.dom).render(JSXdom);
    document.body.appendChild(this.dom);
  },
  warning({ content, duration = DEFAULT_DURATION }) {
    this.dom = document.createElement("div");
    const JSXdom = (
      <Message content={content} duration={duration} type="warning"></Message>
    );
    createRoot(this.dom).render(JSXdom);
    document.body.appendChild(this.dom);
  },
  info({ content, duration = DEFAULT_DURATION }) {
    this.dom = document.createElement("div");
    const JSXdom = (
      <Message content={content} duration={duration} type="warning"></Message>
    );
    createRoot(this.dom).render(JSXdom);
    document.body.appendChild(this.dom);
  },
};

export { message, Message };
