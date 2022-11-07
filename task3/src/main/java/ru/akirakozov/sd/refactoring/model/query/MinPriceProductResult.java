package ru.akirakozov.sd.refactoring.model.query;

import ru.akirakozov.sd.refactoring.model.Product;

public record MinPriceProductResult(Product product) implements QueryResult {
}
