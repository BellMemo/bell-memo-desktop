import * as React from "react";
import {
  Button,
  ButtonGroup,
  ClickAwayListener,
  Grow,
  Paper,
  Popper,
  MenuItem,
  MenuList,
} from "@mui/material";
import { ArrowDropDown } from "@mui/icons-material";
import { invoke } from "@tauri-apps/api";

const options = [
  { id: "sync", name: "同步数据" },
  { id: "config", name: "个性化配置" },
  { id: "export", name: "导出数据" },
  { id: "import", name: "导入数据" },
] as const;

export function Action() {
  const [open, setOpen] = React.useState(false);
  const anchorRef = React.useRef<HTMLDivElement>(null);

  const fetchInfo = async () => {
    const result = await invoke("search_memo_tag", {
      params: {
        content: "",
        offset: 0,
        limit: 10,
      },
    });
    console.log(result);
  };

  const handleClick = async () => {
    console.log(123);
    await fetchInfo();
    await invoke("insert_memo_tag", {
      params: { content: "test tag" },
    });
    await fetchInfo();
  };

  const handleMenuItemClick = (option: typeof options[number]) => {
    setOpen(false);
    console.log(option);
  };

  const handleToggle = () => {
    setOpen((prevOpen) => !prevOpen);
  };

  const handleClose = (event: Event) => {
    if (
      anchorRef.current &&
      anchorRef.current.contains(event.target as HTMLElement)
    ) {
      return;
    }
    setOpen(false);
  };

  return (
    <React.Fragment>
      <ButtonGroup
        variant="contained"
        ref={anchorRef}
        aria-label="split button"
      >
        <Button onClick={handleClick}>新增记录</Button>
        <Button
          size="small"
          aria-controls={open ? "split-button-menu" : undefined}
          aria-expanded={open ? "true" : undefined}
          aria-label={"新增记录"}
          aria-haspopup="menu"
          onClick={handleToggle}
        >
          <ArrowDropDown />
        </Button>
      </ButtonGroup>
      <Popper
        sx={{
          zIndex: 1,
        }}
        open={open}
        anchorEl={anchorRef.current}
        role={undefined}
        transition
        disablePortal
      >
        {({ TransitionProps, placement }) => (
          <Grow
            {...TransitionProps}
            style={{
              transformOrigin:
                placement === "bottom" ? "center top" : "center bottom",
            }}
          >
            <Paper>
              <ClickAwayListener onClickAway={handleClose}>
                <MenuList id="split-button-menu" autoFocusItem>
                  {options.map((option, index) => (
                    <MenuItem
                      key={option.id}
                      onClick={() => handleMenuItemClick(option)}
                    >
                      {option.name}
                    </MenuItem>
                  ))}
                </MenuList>
              </ClickAwayListener>
            </Paper>
          </Grow>
        )}
      </Popper>
    </React.Fragment>
  );
}
