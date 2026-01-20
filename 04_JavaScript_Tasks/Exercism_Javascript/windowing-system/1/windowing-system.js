// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */
export class Size {
  // let width,height;
  constructor(width=80,height=60) {
    this.width = width;
    this.height = height;
  }
  resize(newWidth, newHeight){
    this.width = newWidth;
    this.height = newHeight;
  }
}

export class Position {
  // let x,y;
  constructor(x=0,y=0) {
    this.x=x;
    this.y=y;
  }

  move(newX, newY){
    this.x = newX;
    this.y = newY;
  }
}

export class ProgramWindow  {
  constructor(){
    this.screenSize = new Size(800,600);
    this.size = new Size();
    this.position = new Position();
  }

  resize(newSize){  
    const maxWidth = this.screenSize.width - this.position.x;
    const maxHeight = this.screenSize.height - this.position.y;

    let width = Math.min(maxWidth,Math.max(newSize.width,1));
    let height = Math.min(maxHeight,Math.max(newSize.height,1));
    
    this.size.resize(width,height);
  }

  move(newPosition){
    const maxX = this.screenSize.width - this.size.width;
    const maxY = this.screenSize.height - this.size.height;

    let x = Math.min(maxX,Math.max(0,newPosition.x));
    let y = Math.min(maxY,Math.max(0,newPosition.y));

    this.position.move(x,y);
  }
}

export function changeWindow (programWindow){
  programWindow.resize(new Size(400,300));
  programWindow.move(new Position(100,150));
  
  return programWindow;
}