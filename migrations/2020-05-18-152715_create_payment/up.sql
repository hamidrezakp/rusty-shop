CREATE TABLE "Payment" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"datetime"	TEXT NOT NULL,
	"amount"	REAL NOT NULL DEFAULT 0,
	"order"	INTEGER NOT NULL,
	"customer"	INTEGER NOT NULL,
	FOREIGN KEY("customer") REFERENCES "User"("id") ON UPDATE CASCADE,
	FOREIGN KEY("order") REFERENCES "Order"("id") ON UPDATE CASCADE
)
