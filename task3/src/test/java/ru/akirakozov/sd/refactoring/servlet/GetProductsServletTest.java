package ru.akirakozov.sd.refactoring.servlet;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;
import ru.akirakozov.sd.refactoring.Utils;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.io.PrintWriter;
import java.io.StringWriter;
import java.sql.*;

import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

public class GetProductsServletTest {
    @Mock
    private HttpServletRequest request;
    @Mock
    private HttpServletResponse response;

    private GetProductsServlet servlet;

    @BeforeEach
    void setUp() throws SQLException {
        MockitoAnnotations.openMocks(this);
        servlet = new GetProductsServlet(Utils.getProductServiceForTestDatabase());
        Utils.cleanTestDatabase();
    }

    @Test
    void testGetZeroProducts() throws IOException {
        StringWriter stringWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(stringWriter));

        servlet.doGet(request, response);

        String[] resp = {};
        Assertions.assertArrayEquals(Utils.getCommonHtmlStyling(resp), stringWriter.toString().split("\r\n|\n"));
        verify(response).setStatus(HttpServletResponse.SC_OK);
    }

    @Test
    void testGetTwoProducts() throws IOException, SQLException {
        try (Connection c = Utils.getTestDatabase()) {
            Statement stmt = c.createStatement();
            stmt.executeUpdate("INSERT INTO PRODUCT VALUES (1, 'cookies', 100), (2, 'memes', 300)");
            stmt.close();
        }

        StringWriter stringWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(stringWriter));

        servlet.doGet(request, response);

        String[] resp = {"cookies\t100</br>", "memes\t300</br>"};
        Assertions.assertArrayEquals(Utils.getCommonHtmlStyling(resp), stringWriter.toString().split("\r\n|\n"));
        verify(response).setStatus(HttpServletResponse.SC_OK);
    }
}
