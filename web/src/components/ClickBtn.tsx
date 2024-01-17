import { Button } from "@/components/ui/button";

function ClickBtn() {
  function onClick() {
    console.log("click");
  }

  return (
    <Button className="my-class-name" onClick={onClick}>
      Click
    </Button>
  );
}

export default ClickBtn;
