schema User:
    name: str
    age: int
    message?: str
    data: Data
    labels: { str: }
//    labels: {str: app: name}
    hc: [int]

    check:
        age > 10, "age must > 10"

schema Data:
    id: int
    value: str