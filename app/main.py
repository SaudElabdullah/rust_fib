from fastapi import FastAPI, Request, Response
from rust_fib import rust_fibonacci

app = FastAPI()

@app.get("/py/fib")
async def py_calc_fib(n: int):
    return {"message": fib(n)}


@app.get("/rust/fib")
async def rust_calc_fib(n: int):
    return {"message": rust_fibonacci(n)}


def fib(n):
    if n <= 1:
        return n
    else:
        return fib(n-1) + fib(n-2)
