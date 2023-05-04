import { ClipboardDocumentIcon } from "@heroicons/react/24/outline";
import React from "react";
import { CopyToClipboard } from "react-copy-to-clipboard";

interface CopyProps {
  text: string;
}

export const Copy: React.FC<CopyProps> = (props) => {
  const { text } = props;
  return (
    <CopyToClipboard text={text}>
      <ClipboardDocumentIcon
        className="h-4 w-4 text-gray-500"
        aria-hidden="true"
      />
    </CopyToClipboard>
  );
};
