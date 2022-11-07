package ru.akirakozov.sd.refactoring.servlet;

import ru.akirakozov.sd.refactoring.gateways.Renderer;
import ru.akirakozov.sd.refactoring.service.ProductService;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;

/**
 * @author akirakozov
 */
public class QueryServlet extends HttpServlet {
    protected ProductService productService;
    protected Renderer renderer;

    public QueryServlet(ProductService productService, Renderer renderer) {
        this.productService = productService;
        this.renderer = renderer;
    }

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String command = request.getParameter("command");

        ProductService.QueryType queryType = null;
        if ("max".equals(command)) {
            queryType = ProductService.QueryType.MOST_EXPENSIVE_PRODUCT;
        } else if ("min".equals(command)) {
            queryType = ProductService.QueryType.CHEAPEST_PRODUCT;
        } else if ("sum".equals(command)) {
            queryType = ProductService.QueryType.PRICE_OF_ALL_PRODUCTS;
        } else if ("count".equals(command)) {
            queryType = ProductService.QueryType.PRODUCTS_COUNT;
        }

        String responseData;
        if (queryType != null) {
            responseData = renderer.renderProductQueryResponse(productService.query(queryType));
        } else {
            responseData = renderer.renderUnknownQueryResponse(command);
        }

        response.getWriter().println(responseData);
        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
    }
}
