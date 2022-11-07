package ru.akirakozov.sd.refactoring.model.query;

sealed public interface QueryResult permits MaxPriceProductResult, MinPriceProductResult,
        SumPriceResult, ProductsCountResult {
}

