package ru.akirakozov.sd.refactoring.gateways;

import ru.akirakozov.sd.refactoring.model.query.QueryResult;

public interface Renderer {
    String renderProductQueryResponse(QueryResult result);
    String renderUnknownQueryResponse(String query);
    String renderSuccessfulAddProductResponse();
}
