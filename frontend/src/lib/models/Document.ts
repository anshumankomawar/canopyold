export default class Document {
    id: number;
    name: string;
    content: string;
  
    constructor(id: number, name: string, content: string) {
      this.id = id;
      this.name = name;
      this.content = content;
    }
}  