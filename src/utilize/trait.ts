// 定义 NewsArticle 类，用于表示新闻文章
class NewsArticle {
	public headline: string;
	public location: string;
	public author: string;
	public content: string;

	constructor(
		headline: string,
		location: string,
		author: string,
		content: string
	) {
		this.headline = headline;
		this.location = location;
		this.author = author;
		this.content = content;
	}
}

// 定义 Book 类，用于表示书籍
class Book {
	public title: string;
	public author: string;
	public year: number;

	constructor(title: string, author: string, year: number) {
		this.title = title;
		this.author = author;
		this.year = year;
	}
}

// 定义 Summary 接口，类似于Rust的 trait，规定实现该接口的类必须有 summarize方法
interface Summary {
	summarize(): string;
}

class SummarizableNewsArticle extends NewsArticle implements Summary {
	summarize(): string {
		return `${this.headline}, by ${this.author} (${this.location})`;
	}
}

class SummarizableBook extends Book implements Summary {
	summarize(): string {
		return `${this.title}, by ${this.author} (${this.year})`;
	}
}

// 定义 notify 函数，接受一个实现了 Summary 接口的对象
function notify(item: Summary) {
	console.log(`News! ${item.summarize()}`);
}

function main() {
	let article = new SummarizableNewsArticle(
		"New Study on Climate Change",
		"New York",
		"John Doe",
		"A new study reveals the latest findings on climate change."
	);

	let book = new SummarizableBook(
		"The Rust Programming Language",
		"Steve Klabnik and Carol Nichols",
		2018
	);

	notify(article);
	notify(book);
}

main();
