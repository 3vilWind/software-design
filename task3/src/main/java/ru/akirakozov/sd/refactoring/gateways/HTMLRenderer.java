package ru.akirakozov.sd.refactoring.gateways;

import ru.akirakozov.sd.refactoring.model.*;
import ru.akirakozov.sd.refactoring.model.query.*;

import java.io.PrintWriter;
import java.io.StringWriter;

public class HTMLRenderer implements Renderer {
    public String renderProductQueryResponse(QueryResult result) {
        return switch (result) {
            case MaxPriceProductResult r -> renderProductQueryResult(r.product(), "<h1>Product with max price: </h1>");
            case MinPriceProductResult r -> renderProductQueryResult(r.product(), "<h1>Product with min price: </h1>");
            case SumPriceResult sumPriceResult -> renderIntQueryResult(sumPriceResult.sum(), "Summary price: ");
            case ProductsCountResult productsCountResult ->
                    renderIntQueryResult(productsCountResult.count(), "Number of products: ");
        };
    }

    @Override
    public String renderUnknownQueryResponse(String query) {
        return "Unknown command: " + query;
    }

    protected static String renderProductQueryResult(Product product, String header) {
        return renderIntoBody(new String[]{header, renderProduct(product)});
    }

    protected static String renderIntQueryResult(int result, String header) {
        return renderIntoBody(new String[]{header, String.valueOf(result)});
    }

    protected static String renderProduct(Product product) {
        return product.name + "\t" + product.price + "</br>";
    }

    protected static String renderIntoBody(String[] data) {
        StringWriter stringWriter = new StringWriter();
        PrintWriter printWriter = new PrintWriter(stringWriter);
        printWriter.println("<html><body>");
        for (String line : data) {
            printWriter.println(line);
        }
        printWriter.println("</body></html>");
        return stringWriter.toString();
    }
}
